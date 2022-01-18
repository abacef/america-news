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
        const usernameIsAvailable = this.isUsernameAvailable()
        if (usernameIsAvailable) {
            const user = new User(username, password);
            this.users.push(user);
            return true;
        } else {
            return false;
        }
    }

    isUsernameAvailable(username) {
        return !this.users.map(user => user.username).includes(username);
    }

    passwordIsValid(password) {
        if (password.length < 8) {
            return "The password needs to be 8 or more characters";
        }
        return true;
    }
}

const users = new Users();
export default users;
  