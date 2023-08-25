mod env;

fn main() {
    let env = env::get_env().unwrap();
    println!("Discord token: {}", env.discord_token);
}
