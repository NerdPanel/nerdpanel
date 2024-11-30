import type { LayoutLoad } from './$types';

export const load = (async ({ fetch }) => {
    // TODO fix when cors is enabled
    let data = await fetch('http://localhost:3000/api/user/self', { credentials: 'include' }).then(
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
