interface ChatDetails {
    name: string;
    persona: string;
}

export const getChatDetails = async (chatId: string): Promise<ChatDetails> => {
    const response = await fetch(`http://localhost:8000/api/chat/${chatId}`);
    return response.json();
}