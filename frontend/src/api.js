const API_BASE_URL = "http://localhost:8000";

export const fetchPosts = async (search = "") => {
  const response = await fetch(`${API_BASE_URL}/posts?search=${search}`);
  if (!response.ok) throw new Error("Failed to fetch posts");
  return response.json();
};

export const fetchPostDetail = async (id) => {
  const response = await fetch(`${API_BASE_URL}/posts/${id}`);
  if (!response.ok) throw new Error("Post not found");
  return response.json();
};

export const createPost = async (postData) => {
  const response = await fetch(`${API_BASE_URL}/posts`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(postData),
  });
  return response.json();
};
