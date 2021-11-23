use systemstat::{saturating_sub_bytes, Platform, System};

fn main() {
    let sys = System::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nPontos montados: ");
            for mount in mounts.iter() {
                println!(
                    "{} ---{}---> {} ",
                    mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on
                );
            }
        }
        Err(x) => println!("\nErro de montagem: {}", x),
    }

    match sys.mount_at("/") {
        Ok(mount) => {
            println!("\nSistema montado em:");
            println!(
                "{} ---{}---> {} ",
                mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on
            );
        }
        Err(x) => println!("\nErro de montagem: {}", x),
    }

    match sys.networks() {
        Ok(netifs) => {
            println!("\nPlacas de rede:");
            for netif in netifs.values() {
                println!("{} ", netif.name);
            }
        }
        Err(x) => println!("\nPlacas de rede: Erro: {}", x),
    }

    match sys.battery_life() {
        Ok(battery) => print!(
            "\nBateria: {}%, {}h{}m restando",
            battery.remaining_capacity * 100.0,
            battery.remaining_time.as_secs() / 3600,
            battery.remaining_time.as_secs() % 60
        ),
        Err(x) => print!("\nBateria: erro: {}", x),
    }

    match sys.on_ac_power() {
        Ok(power) => println!(", AC energia: {}", power),
        Err(x) => println!(", AC energia: erro: {}", x),
    }

    match sys.memory() {
        Ok(mem) => println!(
            "\nMemória: {} usados / {}",
            saturating_sub_bytes(mem.total, mem.free),
            mem.total,
        ),
        Err(x) => println!("\nMemória: erro: {}", x),
    }

    match sys.uptime() {
        Ok(uptime) => println!("\nSegundos ligado: {:?}", uptime),
        Err(x) => println!("\nSegundos ligado: Erro: {}", x),
    }

    match sys.boot_time() {
        Ok(boot_time) => println!("\nHorário de boot: {}", boot_time),
        Err(x) => println!("\nHorário de boot: Erro: {}", x),
    }

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nTemperatura da CPU: {}", cpu_temp),
        Err(x) => println!("\nTemperatura da CPU: Erro: {}", x),
    }
}
