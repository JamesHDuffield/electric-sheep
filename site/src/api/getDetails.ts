export const getDetails = async (): Promise<{ name: string }> => {
    const response = await fetch(`http://localhost:8000/api/chat/${data.id}`);
    return response.json();
}