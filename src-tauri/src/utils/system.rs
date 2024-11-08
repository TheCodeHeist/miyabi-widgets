use machine_info::Machine;

pub fn get_basic_system_info() {
  let mut machine = Machine::new();
  let details = machine.system_info();

  println!("OS Name:        {}", details.os_name);
  println!("OS Version:     {}", details.os_version);
  println!("Kernel Version: {}", details.kernel_version);
  println!("Hostname:       {}", details.hostname);
  println!("Distribution:   {}", details.distribution);
  println!("Total Memory:   {}GB", details.memory / 1024 / 1024 / 1024);
  println!("CPU Brand:      {}", details.processor.brand);
  println!("Number of CPUs: {}", details.total_processors);
  println!("Graphics Cards:");
  for gpu in details.graphics {
    println!("  - {}", gpu.name);
    println!("    - Brand:       {}", gpu.brand);
    println!("    - Memory:      {}GB", gpu.memory / 1024 / 1024 / 1024);
    println!("    - Temperature: {}°C", gpu.temperature);
  }
  println!("Disks:");
  for disk in details.disks {
    println!("  - {}", disk.name);
    println!("    - File System Type: {}", disk.fs);
    println!("    - Storage type:     {}", disk.storage_type);
    println!("    - Mount Point:      {}", disk.mount_point);
    println!(
      "    - Total Size:       {}GB",
      disk.size / 1024 / 1024 / 1024
    );
    println!(
      "    - Available Space:  {}GB",
      disk.available / 1024 / 1024 / 1024
    );
  }
  println!("Machine Model:  {}", details.model.unwrap_or_default());
}

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
