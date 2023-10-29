## Database Configuration 

### User Struct
A basic user has the following properties

```rust
#[derive(Debug)]
pub struct User {
    id: String,
    name: String,
    phone: String,
    password: String,
    photo: Option<String>,
}
```

### Groups
A group consists of multiple users who can be invited or join the group. A group must have an admin
who controls the group. A group must have a fixed number of joiners by a set date. The following is a struct that represents it.


```rust
#[derive(Debug)]
pub struct Group {
    id: String,
    name: String,
    total_members: i32,
    admins: Vec<String>
}
```

```rust
#[derive(Debug)]
pub struct GroupMembers {
    id: String,
    group_id: String,
    members: Vec<String> 
}
```



