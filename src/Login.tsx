import React, { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Login(): React.JSX.Element {

    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");

    let nav = useNavigate();

    async function handleSubmit(event: { preventDefault: () => void; }) {
        event.preventDefault();
        const requestHeaders: HeadersInit = new Headers();
        requestHeaders.set('Content-Type', 'application/json');
        const response = await fetch("http://127.0.0.1:8000/users/login", {
            method: "POST",
            credentials: 'include',
            headers: requestHeaders,
            body: JSON.stringify({
                email: email,
                password: password
            })
        })
        response.json().then((i) => {
            alert(i)
            console.log(i)
        })
        if (response.status === 200) {
            console.log("that worked!!!!")
        } else {
            console.log("that didn't work!!!!")
        }
    }

    async function signInWithCookie(event: { preventDefault: () => void; }) {
        event.preventDefault();
        const response = await fetch("http://127.0.0.1:8000/users/login-cookie", {
            method: 'POST',
            credentials: 'include',
        })
        response.json().then((i) => {
            alert(i)
            console.log(i)
            sessionStorage.setItem("token", i.message)
        })
        if (response.status === 200) {
            console.log("that worked!!!!")
            nav("/");
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
            <button onClick={signInWithCookie}>Sign in With Cookie</button>
        </>
    )
}