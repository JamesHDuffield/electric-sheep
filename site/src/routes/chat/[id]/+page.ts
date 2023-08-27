export const ssr = false;

interface Params {
    id: string
}

interface Data {
    id: string;
}

/** @type {import('./$types').PageLoad} */
export function load(args: { params: Params }): Data {
    return {
        id: args.params.id,
    };
}