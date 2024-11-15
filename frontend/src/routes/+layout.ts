import { fetchWithCreds } from '$lib/utils';
import { json } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load = (async () => {
    let data = await fetchWithCreds('http://localhost:3000/api/user/self').then(async (res) => {
        if (res.ok) {
            return await res.json();
        }
        return null;
    });
    return {
        user: data
    };
}) satisfies LayoutLoad;
