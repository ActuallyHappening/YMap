use std::{
  collections::HashMap,
  net::{Ipv4Addr, SocketAddrV4},
};

use crate::{
  prelude::*,
  utils::{
    cmds::{BinaryPath, IntoArgs},
    paths::RemoteFile,
    project_paths::{local::Website, server::ServerPaths},
  },
};

use ::cargo_leptos;
use clap::Parser as _;


