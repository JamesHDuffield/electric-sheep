export interface InterviewResult {
    win: boolean; defective: boolean; defect?: string; attacked: boolean;
}

interface ChatDetails {
    name: string;
    persona: string;
    result?: InterviewResult
}

export const getChatDetails = async (chatId: string): Promise<ChatDetails> => {
    const response = await fetch(`/api/chat/${chatId}`);
    return response.json();
}