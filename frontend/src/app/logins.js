class User {
    constructor(username, password) {
        this.username = username;
        this.password = password;
    }
}

class Users {
    
    constructor() {
        this.users = []; 
    }

    addUser(username, password) {
        const usernameIsUnique = !this.users.map(user => user.username).includes(username);
        if (usernameIsUnique) {
            const user = new User(username, password);
            this.users.push(user);
            return true;
        } else {
            return false;
        }
    }
}

const users = new Users();
export default users;
  