import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}

// TODO fix when cors is enabled
export async function fetchWithCreds(url: string, options?: RequestInit) {
    const defaultOptions: RequestInit = {
        credentials: 'include',
        ...options
    };

    return await fetch(url, defaultOptions);
}
