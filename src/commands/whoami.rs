use crate::user::User;

fn emoji(b: bool) -> String {
    if b {
        "✅".to_string()
    } else {
        "⛔".to_string()
    }
}

pub fn whoami(user: &User) {
    let user = &user.account.data;

    println!("👋 You are logged in as {}.", user.email);
    println!(
        "{} Enterprise | {} Business | {} Pro",
        &emoji(user.has_enterprise_zones),
        &emoji(user.has_business_zones),
        &emoji(user.has_pro_zones)
    );

    if user.has_enterprise_zones {
        println!(
            "🏘️  {} of {} Enterprise Zones are available.",
            user.enterprise_zone_quota.available, user.enterprise_zone_quota.maximum
        );
    }
}
