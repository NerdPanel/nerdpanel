import type { PageLoad } from './$types';

export const load = (async ({ fetch, params }) => {
    let server = await fetch(`api/server/${params.id}`).then((res) => res.json());

    return {
        server
    };
}) satisfies PageLoad;
