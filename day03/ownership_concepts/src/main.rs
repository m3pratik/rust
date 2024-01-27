struct User {
    active: bool,
    sign_in_count: u32,
    username: String,
}

struct CarType(i32, String)

fn main(){
    
    let user1 = User{ active: true, sign_in_count: 1, username: String::from("PratikPanchal")};
    println!("{}",user1.username);

    let user2= create_user(String::from("Hetal"));
    println!("{}",user2.username);
}

fn create_user(username: String) -> User
{
    User {
        username,
        active: true,
        sign_in_count: 1,
    }
}