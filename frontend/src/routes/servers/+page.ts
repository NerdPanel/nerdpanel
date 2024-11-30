import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    let servers = await fetch('http://localhost:3000/api/server', { credentials: 'include' }).then(
        async (res) => {
            if (res.ok) {
                return await res.json();
            }
        }
    );

    return {
        servers
    };
}) satisfies PageLoad;
