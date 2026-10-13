// ============================================================================
//  BOILERPLATE : PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================
//  This boilerplate already compiles and runs. It prints the banner, computes
//  a score from stub numbers, and prints it. Your job is to replace each TODO
//  with a real reading from the sysinfo crate, following the handout, one at a
//  time. Run `cargo run` after each change to be sure it still builds.
//
//  The scoring logic and helpers are done for you. You mostly add println! and
//  set the mem_load and disk_used_pct variables from real data.
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Logical processors : {}", sys.cpus().len());

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let free_mem = total_mem.saturating_sub(used_mem);
    let mem_load: u64 = if total_mem > 0 {
        used_mem.saturating_mul(100) / total_mem
    } else {
        0
    };

    println!("Total memory (GB)  : {:.2}", total_mem as f64 / GB);
    println!("Free memory (GB)   : {:.2}", free_mem as f64 / GB);
    println!("Memory usage       : {}%", mem_load);

    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    let disks = Disks::new_with_refreshed_list();
    let mut disk_used_pct: f64 = 0.0;
    if let Some(disk) = disks.list().first() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total > 0 {
            disk_used_pct = total.saturating_sub(available) as f64 * 100.0 / total as f64;
        }
        println!("Total disk (GB)    : {:.2}", total as f64 / GB);
        println!("Free disk (GB)     : {:.2}", available as f64 / GB);
        println!("Disk usage         : {:.1}%", disk_used_pct);
    } else {
        println!("Disk information   : unavailable");
    }

    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn healthy_system_scores_100() {
        let findings = Findings {
            mem_load: 50,
            disk_used_pct: 50.0,
        };
        assert_eq!(compute_score(&findings), 100);
    }

    #[test]
    fn high_memory_reduces_score() {
        let findings = Findings {
            mem_load: 90,
            disk_used_pct: 50.0,
        };
        assert_eq!(compute_score(&findings), 75);
    }

    #[test]
    fn high_disk_usage_reduces_score() {
        let findings = Findings {
            mem_load: 50,
            disk_used_pct: 95.0,
        };
        assert_eq!(compute_score(&findings), 75);
    }
}
