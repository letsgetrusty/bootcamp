use std::rc::Rc;

struct Database {}

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    let db = Rc::new(Database {});
    let auth_service = AuthService { 
        db: db.clone()
    };
    let content_service = ContentService { 
        db: db.clone()
    };
}
