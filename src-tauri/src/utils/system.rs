use std::{collections::HashMap, net::IpAddr};

use tauri::{async_runtime, App, Emitter};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RamInfo {
  pub total: u64,
  pub used: u64,
  pub free: u64,
  pub avail: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiskInfo {
  pub name: String,
  pub kind: String,
  pub fs: String,
  pub mount_point: String,
  pub total_space: u64,
  pub available_space: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GpuInfo {
  pub id: String,
  pub memory_usage: u32,
  pub encoder_usage: u32,
  pub decoder_usage: u32,
  pub global_utilization: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IpNetworksInfo {
  pub ip: IpAddr,
  pub mask: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NetworkInfo {
  received: u64,
  total_received: u64,
  transmitted: u64,
  total_transmitted: u64,
  packets_received: u64,
  total_packets_received: u64,
  packets_transmitted: u64,
  total_packets_transmitted: u64,
  errors_on_received: u64,
  total_errors_on_received: u64,
  errors_on_transmitted: u64,
  total_errors_on_transmitted: u64,
  mac_address: sysinfo::MacAddr,
  ip_networks: Vec<IpNetworksInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BasicMachineInfo {
  pub os_name: String,
  pub os_version: String,
  pub kernel_version: String,
  pub hostname: String,
  pub distribution: String,
  pub cpu: String,
  pub total_processors: usize,
  pub gpu: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChangingMachineInfo {
  pub cpu_usage: Vec<f32>,
  pub gpu_usage: Vec<GpuInfo>,
  pub ram_usage: RamInfo,
  pub disk_usage: Vec<DiskInfo>,
}

pub fn get_cpu_info(sys: &mut sysinfo::System) -> Vec<f32> {
  sys.refresh_all();

  let mut cpu_usage = Vec::new();
  for cpu in sys.cpus().iter() {
    cpu_usage.push(cpu.cpu_usage());
  }

  cpu_usage
}

pub fn get_gpu_info(machine: &mut machine_info::Machine) -> Vec<GpuInfo> {
  let details = machine.graphics_status();

  let mut gpu_usage = Vec::new();
  for gpu in details {
    gpu_usage.push(GpuInfo {
      id: gpu.id.clone(),
      memory_usage: gpu.memory_usage,
      encoder_usage: gpu.encoder,
      decoder_usage: gpu.decoder,
      global_utilization: gpu.gpu,
    });
  }

  gpu_usage
}

pub fn get_ram_info(sys: &mut sysinfo::System) -> RamInfo {
  sys.refresh_all();

  RamInfo {
    total: sys.total_memory(),
    used: sys.used_memory(),
    free: sys.free_memory(),
    avail: sys.available_memory(),
  }
}

pub fn get_disk_info(partitions: &mut sysinfo::Disks) -> Vec<DiskInfo> {
  partitions.refresh_list();

  let mut disks = Vec::new();

  for partition in partitions.iter() {
    disks.push(DiskInfo {
      name: partition.name().to_os_string().into_string().unwrap(),
      kind: partition.kind().to_string(),
      fs: partition
        .file_system()
        .to_os_string()
        .into_string()
        .unwrap_or_else(|e| panic!("Failed to parse OsString to String: {:?}", e)),
      mount_point: partition
        .mount_point()
        .to_str()
        .unwrap_or_else(|| panic!("Failed to parse OsString to &str..."))
        .to_string(),
      total_space: partition.total_space(),
      available_space: partition.available_space(),
    });
  }

  disks
}

pub fn get_network_info(net: &mut sysinfo::Networks) -> HashMap<String, NetworkInfo> {
  net.refresh_list();

  let mut network_info = HashMap::new();

  for (interface, network) in net.iter() {
    let mut ip_networks = Vec::new();

    for ip in network.ip_networks() {
      ip_networks.push(IpNetworksInfo {
        ip: ip.clone().addr,
        mask: ip.clone().prefix,
      });
    }

    network_info.insert(
      interface.clone(),
      NetworkInfo {
        received: network.received(),
        total_received: network.total_received(),
        transmitted: network.transmitted(),
        total_transmitted: network.total_transmitted(),
        packets_received: network.packets_received(),
        total_packets_received: network.total_packets_received(),
        packets_transmitted: network.packets_transmitted(),
        total_packets_transmitted: network.total_packets_transmitted(),
        errors_on_received: network.errors_on_received(),
        total_errors_on_received: network.total_errors_on_received(),
        errors_on_transmitted: network.errors_on_transmitted(),
        total_errors_on_transmitted: network.total_errors_on_transmitted(),
        mac_address: network.mac_address(),
        ip_networks,
      },
    );
  }

  network_info
}

pub fn get_basic_machine_info() -> BasicMachineInfo {
  let mut machine = machine_info::Machine::new();

  let details = machine.system_info();

  BasicMachineInfo {
    os_name: details.os_name.trim().to_string(),
    os_version: details.os_version.trim().to_string(),
    kernel_version: details.kernel_version.trim().to_string(),
    hostname: details.hostname.trim().to_string(),
    distribution: details.distribution.trim().to_string(),
    cpu: details.processor.brand.trim().to_string(),
    total_processors: details.total_processors,
    gpu: details
      .graphics
      .iter()
      .map(|g| g.name.clone().trim().to_string())
      .collect(),
  }
}

pub fn get_changing_machine_info() -> ChangingMachineInfo {
  let mut sys = sysinfo::System::new_all();
  let mut machine = machine_info::Machine::new();
  let mut partitions = sysinfo::Disks::new();

  ChangingMachineInfo {
    cpu_usage: get_cpu_info(&mut sys),
    gpu_usage: get_gpu_info(&mut machine),
    ram_usage: get_ram_info(&mut sys),
    disk_usage: get_disk_info(&mut partitions),
  }
}

pub fn initiate_system_info_fetcher(app: &App) -> Result<(), String> {
  let app_handle = app.handle().clone();

  let basic_machine_info = get_basic_machine_info();

  app_handle
    .emit("basicMachineInfo", basic_machine_info)
    .unwrap_or_else(|e| {
      eprintln!("Failed to emit basic machine info event: {}", e);
    });

  // println!(
  //   "{}",
  //   serde_json::to_string_pretty(&basic_machine_info).unwrap()
  // );

  let app_handle_clone1 = app_handle.clone();

  async_runtime::spawn(async move {
    loop {
      let changing_machine_info = get_changing_machine_info();

      app_handle_clone1
        .emit("changingMachineInfo", changing_machine_info.clone())
        .unwrap_or_else(|e| {
          eprintln!("Failed to emit changing machine info event: {}", e);
        });

      // println!(
      //   "{}",
      //   serde_json::to_string_pretty(&changing_machine_info).unwrap()
      // );

      std::thread::sleep(std::time::Duration::from_secs(1));
    }
  });

  let app_handle_clone2 = app_handle.clone();

  async_runtime::spawn(async move {
    let mut net = sysinfo::Networks::new_with_refreshed_list();

    loop {
      std::thread::sleep(std::time::Duration::from_secs(1));

      let network_info = get_network_info(&mut net);

      app_handle_clone2
        .emit("networkInfo", network_info.clone())
        .unwrap_or_else(|e| {
          eprintln!("Failed to emit network info event: {}", e);
        });

      // println!("{}", serde_json::to_string_pretty(&network_info).unwrap());
    }
  });

  Ok(())
}

// pub fn get_cpu_usage() {
//   let mut sys = sysinfo::System::new_all();
//   sys.refresh_all();

//   println!("Global CPU Usage: {}%", sys.global_cpu_usage());
//   for (i, cpu) in sys.cpus().iter().enumerate() {
//     println!("CPU{} Usage: {}%", i, cpu.cpu_usage());
//   }
// }

// pub fn get_ram_usage() {
//   let mut sys = sysinfo::System::new_all();
//   sys.refresh_all();

//   let used = sys.used_memory();
//   let total = sys.total_memory();
//   let free = sys.free_memory();
//   let avail = sys.available_memory();

//   println!(
//     "RAM Usage: {}GB/{}GB ({}GB), available: {}GB",
//     used / 1024 / 1024 / 1024,
//     total / 1024 / 1024 / 1024,
//     free / 1024 / 1024 / 1024,
//     avail / 1024 / 1024 / 1024
//   );
// }

// pub fn get_temperature() {
// let plat: systemstat::platform::PlatformImpl = systemstat::Platform::new();
// let temp = plat
//   .cpu_temp()
//   .unwrap_or_else(|e| panic!("Failed to get CPU temperature: {}", e));

// println!("CPU Temperature: {}°C", temp);

//   let components = sysinfo::Components::new_with_refreshed_list();

//   if components.is_empty() {
//     println!("There is no temperature information available.");
//   } else {
//     for component in &components {
//       println!("{}: {}°C", component.label(), component.temperature());
//     }
//   }
// }

// Information required to get system information
// - CPU load, temperature, clock speed and fan speed
// - GPU load, temperature, clock speed and fan speed
// - RAM usage
// - Network upload and download speed
// - Disk usage
// - Top processes
// Use the following crates to get the information:
// - sysinfo
// - machine-info

// use sysinfo::System;

// #[derive(Debug)]
// pub struct BasicSystemInfo {
//   pub os: String,
//   pub hostname: String,
//   pub kernel_version: String,
//   pub uptime: u64,
//   pub nb_cpus: u32,
//   pub cpu_brand: String,
//   pub cpu_freq: u64,
//   pub cpu_usage: f32,
//   pub gpu_brand: String,
//   pub total_memory: u64,
//   pub free_memory: u64,
//   pub used_memory: u64,
//   pub total_swap: u64,
//   pub free_swap: u64,
//   pub used_swap: u64,
// }

// pub fn get_basic_system_info() -> BasicSystemInfo {
//   let mut sys = System::new_all();
//   sys.refresh_all();

//   BasicSystemInfo {
//     os: System::long_os_version()
//       .unwrap_or_default()
//       .trim()
//       .to_string(),
//     hostname: System::host_name().unwrap_or_default().trim().to_string(),
//     kernel_version: System::kernel_version()
//       .unwrap_or_default()
//       .trim()
//       .to_string(),
//     uptime: System::uptime(),
//     nb_cpus: sys.cpus().len() as u32,
//     cpu_brand: sys.cpus()[0].brand().to_string().trim().to_string(),
//     cpu_freq: sys.cpus()[0].frequency(),
//     cpu_usage: sys.global_cpu_usage(),
//     total_memory: sys.total_memory(),
//     free_memory: sys.free_memory(),
//     used_memory: sys.used_memory(),
//     total_swap: sys.total_swap(),
//     free_swap: sys.free_swap(),
//     used_swap: sys.used_swap(),
//   }
// }

// pub struct RamUsage {
//   pub total: u64,
//   pub used: u64,
//   pub free: u64,
//   pub avail: u64,
// }

// pub fn get_ram_usage() {
//   let mut sys = sysinfo::System::new_all();
//   sys.refresh_all();
//   let used = sys.used_memory();
//   let total = sys.total_memory();
//   let free = sys.free_memory();
//   let avail = sys.available_memory();

//   println!(
//     "RAM Usage: {}GB/{}GB ({}GB), available: {}GB",
//     used / 1024 / 1024 / 1024,
//     total / 1024 / 1024 / 1024,
//     free / 1024 / 1024 / 1024,
//     avail / 1024 / 1024 / 1024
//   );
// }

// pub fn get_cpu_usage() {
//   let mut sys = sysinfo::System::new_all();
//   sys.refresh_all();
//   println!("CPU Usage: {}%", sys.global_cpu_usage());
// }

// pub fn get_temperature() {
//   let components = sysinfo::Components::new_with_refreshed_list();
//   for component in &components {
//     println!("{}: {}°C", component.label(), component.temperature());
//   }
// }
