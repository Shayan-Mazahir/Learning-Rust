// Complete the TODO sections to build a security monitoring system

const MAX_LOGIN_ATTEMPTS: u32 = 3;
const ADMIN_PRIVILEGES: &str = "ADMIN";


fn main() {
    // let warning_threshold = MAX_LOGIN_ATTEMPTS - 1;
    println!("=== Security Monitoring System ===\n");
    
    println!("Challenge 1: User Authentication Tracking");
    // TODO: Create an immutable variable 'username' with value "alice"
    let username = "alice";
    
    // TODO: Create a mutable variable 'failed_attempts' starting at 0
    let mut failed_attempts = 0;
    
    // TODO: Create a mutable variable 'is_locked' starting as false
    let mut is_locked = false;
    
    println!("User: {}", username);
    println!("Failed attempts: {}", failed_attempts);
    println!("Account locked: {}", is_locked);
    
    // Simulate failed login attempts
    // TODO: Increment failed_attempts by 1
      failed_attempts += 1;
    
    // TODO: Increment failed_attempts by 1 again
      failed_attempts += 1;
      failed_attempts += 1;
    
    // TODO: Set is_locked to true when failed_attempts equals MAX_LOGIN_ATTEMPTS
    if failed_attempts == MAX_LOGIN_ATTEMPTS {
        is_locked = true;
    }

    
    println!("\nAfter failed logins:");
    println!("Failed attempts: {}", failed_attempts);
    println!("Account locked: {}", is_locked);
    
    println!("\n=== Challenge 2: Permission Escalation Detection ===");
    
    // TODO: Create a variable 'user_role' with initial value "USER"
    let user_role = "USER";
    
    println!("Initial role: {}", user_role);
    
    // Simulate privilege escalation attempt
    {
        // TODO: Shadow user_role to temporarily show "ATTEMPTING_ESCALATION"
        let user_role = "ATTEMPTING_ESCALATION";
        
        println!("Security alert: {}", user_role);
        
        // TODO: Create a variable 'threat_level' that only exists in this scope with value "HIGH"
        let threat_level = "HIGH";
        
        println!("Threat level: {}", threat_level);
    }
    
    // TODO: Print user_role here - what should it be now?
    println!("Role after security check: {}", user_role);
    
    println!("\n=== Challenge 3: Password Security Levels ===");
    
    // TODO: Create a variable 'password_strength' starting with numeric value 2
    let password_strength = 2;
    
    println!("Initial password strength (1-5): {}", password_strength);
    
    // TODO: Shadow password_strength to increase it to 4
    let password_strength = 4;
    
    println!("After adding special characters: {}", password_strength);
    
    // TODO: Shadow password_strength again to convert to descriptive string "STRONG"
    // password_strength.to_string();
    let password_strength = "STRONG";
    
    println!("Security rating: {}", password_strength);
    
    println!("\n=== Challenge 4: Firewall Rule Management ===");
    
    // TODO: Create a mutable variable 'blocked_ips' starting at 0
    let mut blocked_ips = 0;
    
    println!("Currently blocked IPs: {}", blocked_ips);
    
    // TODO: Increment blocked_ips to 5 (simulating blocking attackers)
     blocked_ips += 5;
    
    // TODO: Increment blocked_ips to 12 (more attacks detected)
     blocked_ips += 12;
    
    println!("Blocked IPs after attack wave: {}", blocked_ips);
    
    // TODO: Try to change blocked_ips to string "MANY" - this should cause an error
    // Comment out after testing:
    // blocked_ips = "MANY";
    
    // blocked_ips.to_string();
    // let mut blocked_ips = "MANY";

    // TODO: Instead, shadow blocked_ips with the string "CRITICAL_LEVEL"
    // blocked_ips.to_string();
    let blocked_ips = "Critical Level";
    
    println!("Alert status: {}", blocked_ips);
    
    println!("\n=== Challenge 5: Security Audit Log ===");
    
    // TODO: Create a variable 'log_entry' with value "LOGIN_SUCCESS"
    let log_entry = "LOGIN SUCCESS";
    
    println!("Log: {}", log_entry);
    
    // Simulate suspicious activity detection
    {
        // TODO: Shadow log_entry to "SUSPICIOUS_ACTIVITY_DETECTED"
        let log_entry = "SUSPICIOUS ACTIVITY DETECTED";
        
        // TODO: Create an 'alert_code' variable with value 4001
        let alert_code = 4001;
        
        println!("Security log: {} (Code: {})", log_entry, alert_code);
        
        {
            // TODO: Shadow log_entry again to "INITIATING_LOCKDOWN"
            let log_entry = "INITIATING LOCKDOWN";
            
            // TODO: Shadow alert_code to 9999
            let alert_code = 9999;
            
            println!("CRITICAL: {} (Code: {})", log_entry, alert_code);
        }
        
        // TODO: Print log_entry and alert_code here - what values should they have?
        println!("After inner scope: {} (Code: {})", log_entry, alert_code);
    }
    
    // TODO: Print log_entry here - what should it be?
    println!("Final log state: {}", log_entry);
    
    println!("\n=== Challenge 6: Constants in Security ===");
    
    println!("System configured for max {} login attempts", MAX_LOGIN_ATTEMPTS);
    println!("Admin privileges: {}", ADMIN_PRIVILEGES);
    
    // TODO: Try to modify MAX_LOGIN_ATTEMPTS - this should error
    // Comment out after testing:
    // MAX_LOGIN_ATTEMPTS = 5;
    // let MAX_LOGIN_ATTEMPTS = 5;
    
    // TODO: Create a calculation using MAX_LOGIN_ATTEMPTS
    // Shadow a new variable 'warning_threshold' that's 1 less than MAX_LOGIN_ATTEMPTS
    let warning_threshold = MAX_LOGIN_ATTEMPTS - 1;
    
    println!("Warning threshold: {}", warning_threshold);
    
    println!("\n=== System Status ===");
    println!("Security monitoring active ✓");
}

/*
NEEDA DO THIS AGAIN
CYBERSECURITY CONCEPTS COVERED:
- Authentication state tracking (mutable vs immutable)
- Scope-based security contexts
- Permission level transformations
- Threat level escalation
- Audit trail management
- Security configuration constants

RUST CONCEPTS PRACTICED:
- let vs let mut
- Variable shadowing
- Scope boundaries
- Type transformations through shadowing
- Constant usage
- Compile-time safety

DEBUGGING TIPS:
- Uncomment error lines to see Rust's helpful error messages
- Pay attention to which variables can be modified vs shadowed
- Notice how scope affects security contexts
- Observe how constants enforce security policies
*/