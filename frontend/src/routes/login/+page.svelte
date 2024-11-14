<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";

    function login() {
        const username = (document.getElementById('username') as HTMLInputElement).value;
        const password = (document.getElementById('password') as HTMLInputElement).value;

        if (username && password) {
            // Perform login logic here, e.g., send a request to the server
            fetch('http://localhost:3000/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ username, password }),
            }).then((response) => {
                    if (response.ok) {
                        // TODO Redirect the user to the dashboard page
                        // window.location.href = '/dashboard';
                    } else {
                        // Show an error message to the user
                        console.error('Invalid username or password');
                    }
                })
                .catch((error) => {
                    // Show an error message to the user
                    console.error('An error occurred while logging in', error);
                });
        } else {
            // Show an error message to the user
            console.error('Please enter your username and password');
        }
    }
  </script>
  
<Card.Root class="flex justify-center max-w-sm flex-col">
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
  