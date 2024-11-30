import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    let servers = await fetch('api/server').then(
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
