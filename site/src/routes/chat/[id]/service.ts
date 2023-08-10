export interface InterviewResult {
    win: boolean; defective: boolean; defect?: string
}

interface ChatDetails {
    name: string;
    persona: string;
}

export const getChatDetails = async (chatId: string): Promise<ChatDetails> => {
    const response = await fetch(`/api/chat/${chatId}`);
    return response.json();
}