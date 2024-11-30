import type { LayoutLoad } from './$types';

export const load = (async ({ fetch }) => {
    // TODO fix when cors is enabled
    let data = await fetch('api/user/self').then(
        async (res) => {
            if (res.ok) {
                return await res.json();
            }
            return null;
        }
    );
    return {
        user: data
    };
}) satisfies LayoutLoad;
