const API_BASE_URL = "http://localhost:3000";

const getAuthHeader = () => {
  const token = localStorage.getItem("token");
  return token ? { Authorization: `Bearer ${token}` } : {};
};

export const fetchPosts = async (search = "", category = "") => {
  const response = await fetch(`${API_BASE_URL}/posts?search=${search}&category=${category}`);
  if (!response.ok) throw new Error("Failed to fetch posts");
  return response.json();
};

export const fetchPostDetail = async (id) => {
  const response = await fetch(`${API_BASE_URL}/posts/${id}`);
  if (!response.ok) throw new Error("Post not found");
  return response.json();
};

export const login = async (username, password) => {
  const response = await fetch(`${API_BASE_URL}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  if (!response.ok) throw new Error("Login failed");
  return response.json();
};

export const register = async (username, password) => {
  const response = await fetch(`${API_BASE_URL}/auth/register`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });
  if (!response.ok) throw new Error("Registration failed");
  return response.status === 201;
};

export const createPost = async (postData) => {
  const response = await fetch(`${API_BASE_URL}/admin/posts`, {
    method: "POST",
    headers: { 
      "Content-Type": "application/json",
      ...getAuthHeader()
    },
    body: JSON.stringify(postData),
  });
  if (!response.ok) throw new Error("Failed to create post");
  return response.json();
};

export const updatePost = async (id, postData) => {
  const response = await fetch(`${API_BASE_URL}/admin/posts/${id}`, {
    method: "PATCH",
    headers: { 
      "Content-Type": "application/json",
      ...getAuthHeader()
    },
    body: JSON.stringify(postData),
  });
  if (!response.ok) throw new Error("Failed to update post");
  return response.json();
};

export const deletePost = async (id) => {
  const response = await fetch(`${API_BASE_URL}/admin/posts/${id}`, {
    method: "DELETE",
    headers: getAuthHeader(),
  });
  if (!response.ok) throw new Error("Failed to delete post");
  return response.status === 204;
};
