/**
 * Shared wire types. These mirror the backend `dto.rs` structs 1:1 — when a
 * backend DTO changes, change it here in the same PR.
 */

export type Page<T> = {
  items: T[];
  next_cursor: string | null;
};

export type PostAuthor = {
  id: string;
  username: string;
  display_name: string | null;
  avatar_url: string | null;
};

export type PostView = {
  id: string;
  caption: string;
  image_url: string;
  created_at: string;
  author: PostAuthor;
  like_count: number;
  comment_count: number;
  liked_by_me: boolean;
};

export type CommentView = {
  id: string;
  body: string;
  created_at: string;
  author: { id: string; username: string; avatar_url: string | null };
  can_delete: boolean;
};

export type ProfileCounts = {
  posts: number;
  followers: number;
  following: number;
};

export type ProfileResponse = {
  id: string;
  username: string;
  display_name: string | null;
  bio: string;
  avatar_url: string | null;
  created_at: string;
  counts: ProfileCounts;
  is_following: boolean | null;
  is_me: boolean;
};

export type UserSummary = {
  id: string;
  username: string;
  display_name: string | null;
  avatar_url: string | null;
};

export type PresignedUpload = {
  url: string;
  method: "PUT";
  headers: [string, string][];
  key: string;
  expires_in: number;
};
