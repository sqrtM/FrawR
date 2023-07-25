import React, { useState } from "react";

export default function Login(): React.JSX.Element {

    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");

    async function handleSubmit(event: { preventDefault: () => void; }) {
        event.preventDefault();
        const response = await fetch("http://127.0.0.1:8000/api/user", {
            method: "POST",
            body: JSON.stringify({
                email: email,
                password: password
            })
        })
        response.json().then((i) => {
            alert(i)
            console.log(i)
        })
        if (response.status === 202) {
            console.log("that worked!!!!")
        } else {
            console.log("that didn't work!!!!")
        }
    }

    return (
        <>
            <form action="">
                <input type="email" name="email" id="emailinput" onChange={(e) => setEmail(e.target.value)} />
                <input type="password" name="password" id="passwordinput" autoComplete="current password" onChange={(e) => setPassword(e.target.value)} />
                <button type="submit" onClick={handleSubmit}>submit</button>
            </form>
        </>
    )
}