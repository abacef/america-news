
export async function getGreeting() {
    const res = await fetch('http://localhost:8000/api/hello');
    const content = await res.json();

    if (res.ok) {
        return content.greeting;
    } else {
        throw new Error(body);
    }
}