fn main() {
  console_error_panic_hook::set_once();
  utils::tracing::install_tracing(
    "debug,app=trace,mathquill-leptos=trace,mathquill-js=trace,mathquill-js-sys=trace,latex-parser=trace,cas=trace",
  ).unwrap();

  tracing::info!("Initialized logging on front-end");

  app::main::main()
}
