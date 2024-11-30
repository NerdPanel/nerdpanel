import type { PageLoad } from './$types';

export const load = (async ({ fetch, params }) => {
    let server = await fetch(`http://localhost:3000/api/server/${params.id}`, {
        credentials: 'include'
    }).then((res) => res.json());

    return {
        server
    };
}) satisfies PageLoad;
