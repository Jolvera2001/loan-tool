import { Ref } from "vue";

/**
 * A function that performs asynchronous work
 * @template TArgs - Tuple of argument types the function accepts
 * @template TReturn - The type the returned Promise resolves to
 */
export type AsyncFunction<TArgs extends any[] = any[], TReturn = any> = (
    ...args: TArgs
) => Promise<TReturn>;

/**
 * Wraps an async ref so a loading ref toggles when running around its execution
 * 
 * @template T - The async function type to be wrapped
 * @param loading - The ref set to true while `fn` is running
 * @param fn - The Async function to wrap
 * @returns A new function with the same signature as `fn`
 * 
 * @example
 * const isLoading = ref(false)
 * const fetchUser = useLoading(isLoading, getUserById) // Some Async function
 * await fetchUser('123') // if async function accepts arguments
 */
export function useLoading<T extends AsyncFunction>(
    loading: Ref<boolean>,
    fn: T,
): T {
    return (async (...args: Parameters<T>) => {
        loading.value = true;
        try {
            return await fn(...args);
        } finally {
            loading.value = false;
        }
    }) as T;
}
