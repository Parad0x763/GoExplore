export type User = {
    userName: string;
    email: string;
}

export function printUser(user: User) {
    console.log(`User Name: ${user.userName}\nEmail: ${user.email}`);
}