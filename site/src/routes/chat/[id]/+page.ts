import type { MetaTagsProps } from 'svelte-meta-tags'; // Import type for meta tags properties.

export const ssr = false;

interface Params {
    id: string
}

interface Data {
    id: string;
    metaTagsChild: MetaTagsProps,
}

/** @type {import('./$types').PageLoad} */
export function load(args: { params: Params, url: URL }): Data {
    console.log(args)
    const title = 'Android Suspect';
    const description = 'Another potentially defective android, interview them and pass your verdict.';
    const image = `${args.url.origin}/avatars/1.png`;
    const metaTags: MetaTagsProps = {
        title,
        description,
        openGraph: {
            url: args.url.toString(),
            title,
            description,
            images: [
                {
                    url: image,
                },
            ],
        },
        twitter: {
            title,
            cardType: 'summary_large_image',
            description,
            image,
        },
    };
    return {
        id: args.params.id,
        metaTagsChild: metaTags,
    };
}