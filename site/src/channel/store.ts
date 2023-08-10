import { writable } from 'svelte/store';

export interface ChatMessage {
    role: 'assistant' | 'user';
    content: string;
}

export const createChannelStore = (chat_id: string) => {
    const { subscribe, set, update } = writable<ChatMessage[]>([]);

    const eventSource = new EventSource(`/api/join/${chat_id}`);

    eventSource.onmessage = event => {
        update((messages) => messages.concat(JSON.parse(event.data)));
    };

    return {
        subscribe,
        reset: () => set([]),
        close: () => eventSource.close(),
    };
};