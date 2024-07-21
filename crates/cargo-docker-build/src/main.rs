use clap::Parser;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::fmt::init();
	let opts = cargo_docker_build::Opts::parse();
	let docker = cargo_docker_build::new_docker()?;

	match opts.subcmd {
		Cmd::Attach { id } => {
			let container = docker.containers().get(&id);
			let tty_multiplexer = container.attach().await?;

			let (mut reader, _writer) = tty_multiplexer.split();

			while let Some(tty_result) = reader.next().await {
				match tty_result {
					Ok(chunk) => print_chunk(chunk),
					Err(e) => eprintln!("Error: {e}"),
				}
			}
		}
		Cmd::CopyFrom {
			id,
			remote_path,
			local_path,
		} => {
			use futures::TryStreamExt;
			use tar::Archive;
			let bytes = docker
				.containers()
				.get(&id)
				.copy_from(&remote_path)
				.try_concat()
				.await?;

			let mut archive = Archive::new(&bytes[..]);
			archive.unpack(&local_path)?;
		}
		Cmd::CopyInto {
			local_path,
			id,
			remote_path,
		} => {
			use std::{fs::File, io::Read};

			let mut file = File::open(&local_path)?;
			let mut bytes = Vec::new();
			file
				.read_to_end(&mut bytes)
				.expect("Cannot read file on the localhost.");

			if let Err(e) = docker
				.containers()
				.get(&id)
				.copy_file_into(remote_path, &bytes)
				.await
			{
				eprintln!("Error: {e}")
			}
		}
		Cmd::Commit {
			id,
			repo,
			tag,
			comment,
			author,
			pause,
			changes,
		} => {
			use docker_api::opts::ContainerCommitOpts;

			let mut opts = ContainerCommitOpts::builder();

			if let Some(repo) = repo {
				opts = opts.repo(repo)
			}
			if let Some(tag) = tag {
				opts = opts.tag(tag)
			}
			if let Some(comment) = comment {
				opts = opts.comment(comment)
			}
			if let Some(author) = author {
				opts = opts.author(author)
			}
			if let Some(pause) = pause {
				opts = opts.pause(pause)
			}
			if let Some(changes) = changes {
				opts = opts.changes(changes)
			}
			match docker
				.containers()
				.get(id)
				.commit(&opts.build(), None)
				.await
			{
				Ok(id) => println!("{id:?}"),
				Err(e) => eprintln!("Error: {e}"),
			}
		}
		Cmd::Create { image, nam } => {
			use docker_api::opts::ContainerCreateOpts;
			let opts = if let Some(name) = nam {
				ContainerCreateOpts::builder()
					.image(image)
					.name(name)
					.build()
			} else {
				ContainerCreateOpts::builder().image(image).build()
			};
			match docker.containers().create(&opts).await {
				Ok(info) => println!("{info:?}"),
				Err(e) => eprintln!("Error: {e}"),
			}
		}
		Cmd::Delete { id, force } => {
			use docker_api::opts::ContainerRemoveOpts;

			let opts = if force {
				ContainerRemoveOpts::builder().force(true).build()
			} else {
				Default::default()
			};
			if let Err(e) = docker.containers().get(&id).remove(&opts).await {
				eprintln!("Error: {e}")
			}
		}
		Cmd::Exec { id, cmd } => {
			use docker_api::opts::ExecCreateOpts;
			let options = ExecCreateOpts::builder()
				.command(cmd)
				.attach_stdout(true)
				.attach_stderr(true)
				.build();

			let container = docker.containers().get(&id);
			let mut stream = container
				.exec(&options, &Default::default())
				.await
				.expect("exec stream");
			while let Some(exec_result) = stream.next().await {
				match exec_result {
					Ok(chunk) => print_chunk(chunk),
					Err(e) => eprintln!("Error: {e}"),
				}
			}
		}
		Cmd::Inspect { id } => {
			match docker.containers().get(&id).inspect().await {
				Ok(container) => println!("{container:#?}"),
				Err(e) => eprintln!("Error: {e}"),
			};
		}
		Cmd::List { all } => {
			use docker_api::opts::ContainerListOpts;

			let opts = if all {
				ContainerListOpts::builder().all(true).build()
			} else {
				Default::default()
			};
			match docker.containers().list(&opts).await {
				Ok(containers) => {
					containers.into_iter().for_each(|container| {
						println!(
							"{}\t{}\t{:?}\t{}\t{}",
							&container.id.unwrap_or_default()[..12],
							container.image.unwrap_or_default(),
							container.state,
							container.status.unwrap_or_default(),
							container.names.map(|n| n[0].to_owned()).unwrap_or_default()
						);
					});
				}
				Err(e) => eprintln!("Error: {e}"),
			}
		}
		Cmd::Logs { id, stdout, stderr } => {
			use docker_api::opts::LogsOpts;
			let container = docker.containers().get(&id);
			let logs_stream = container.logs(&LogsOpts::builder().stdout(stdout).stderr(stderr).build());

			let logs: Vec<_> = logs_stream
				.map(|chunk| match chunk {
					Ok(chunk) => chunk.to_vec(),
					Err(e) => {
						eprintln!("Error: {e}");
						vec![]
					}
				})
				.collect::<Vec<_>>()
				.await
				.into_iter()
				.flatten()
				.collect::<Vec<_>>();
			print!("{}", String::from_utf8_lossy(&logs));
		}
		Cmd::Prune { until } => {
			use docker_api::opts::{ContainerPruneFilter, ContainerPruneOpts};

			let opts = if let Some(until) = until {
				ContainerPruneOpts::builder()
					.filter(vec![ContainerPruneFilter::Until(until)])
					.build()
			} else {
				Default::default()
			};

			if let Err(e) = docker.containers().prune(&opts).await {
				eprintln!("Error: {e}")
			}
		}
		Cmd::StatFile { id, path } => {
			let stats = docker.containers().get(&id).stat_file(path).await?;
			println!("{stats}");
		}
		Cmd::Stats { id } => {
			while let Some(result) = docker.containers().get(&id).stats().next().await {
				match result {
					Ok(stat) => println!("{stat:?}"),
					Err(e) => eprintln!("Error: {e}"),
				}
			}
		}
		Cmd::Top { id, psargs } => {
			match docker.containers().get(&id).top(psargs.as_deref()).await {
				Ok(top) => println!("{top:#?}"),
				Err(e) => eprintln!("Error: {e}"),
			};
		}
		Cmd::Stop { id, wait, signal } => {
			use docker_api::opts::ContainerStopOpts;

			let mut opts = ContainerStopOpts::builder();
			if let Some(w) = wait {
				opts = opts.wait(std::time::Duration::from_secs(w as u64));
			}
			if let Some(s) = signal {
				opts = opts.signal(s);
			}

			match docker.containers().get(&id).stop(&opts.build()).await {
				Ok(_) => println!("Container {id} stopped..."),
				Err(e) => eprintln!("Error: {e}"),
			};
		}
		Cmd::Restart { id, wait, signal } => {
			use docker_api::opts::ContainerRestartOpts;

			let mut opts = ContainerRestartOpts::builder();
			if let Some(w) = wait {
				opts = opts.wait(std::time::Duration::from_secs(w as u64));
			}
			if let Some(s) = signal {
				opts = opts.signal(s);
			}

			match docker.containers().get(&id).restart(&opts.build()).await {
				Ok(_) => println!("Container {id} restarted..."),
				Err(e) => eprintln!("Error: {e}"),
			};
		}
	}

	Ok(())
}
