<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import { Label } from '$lib/components/ui/label/index.js';

    function login() {
        const username = (document.getElementById('username') as HTMLInputElement).value;
        const password = (document.getElementById('password') as HTMLInputElement).value;

        if (username && password) {
            fetch('api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ username, password }),
            })
                .then((response) => {
                    if (response.ok) {
                        const urlParams = new URLSearchParams(window.location.search);
                        const next = urlParams.get('next');
                        window.location.href = next ? next : '/';
                    } else {
                        // TODO proper alerts
                        alert('Invalid username or password');
                    }
                })
                .catch((error) => {
                    // TODO proper alerts
                    alert('An error occurred while logging in');
                    console.error(error);
                });
        } else {
            // TODO proper alerts
            alert('Please enter a username and password');
        }
    }
</script>

<div class="flex flex-1 items-center justify-center align-middle">
    <Card.Root class="flex max-w-sm flex-col justify-center">
        <Card.Header>
            <Card.Title class="text-2xl">Login</Card.Title>
            <Card.Description>Enter your username below to login to your account.</Card.Description>
        </Card.Header>
        <Card.Content class="grid gap-4">
            <div class="grid gap-2">
                <Label for="username">username</Label>
                <Input id="username" type="username" required />
            </div>
            <div class="grid gap-2">
                <Label for="password">Password</Label>
                <Input id="password" type="password" required />
            </div>
        </Card.Content>
        <Card.Footer>
            <Button class="w-full" onclick={login}>Sign in</Button>
        </Card.Footer>
    </Card.Root>
</div>
