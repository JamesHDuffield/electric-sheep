import { writable, type Writable } from 'svelte/store';
import ReconnectingEventSource from 'reconnecting-eventsource';

export interface ChatMessage {
    role: 'assistant' | 'user';
    content: string;
}

export const createChatStore = (chat_id: string): Writable<ChatMessage[]> => {
    const store = writable<ChatMessage[]>([]);

    const eventSource = new ReconnectingEventSource(`/api/join/${chat_id}`);

    eventSource.onmessage = (event) => {
        store.update((messages) => messages.concat(JSON.parse(event.data)));
    };

    eventSource.onopen = () => {
        console.info('SSE connection opened');
        store.set([]);
    };

    eventSource.onerror = (error) => {
        console.error('SSE connection error', error);
    }

    return store;
};