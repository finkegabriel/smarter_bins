// src/routes/users/[id]/+page.js
export function load({ params }) {
    const { id } = params;
    // Fetch user data based on the id
    return {
      userId: id,
      // userData: fetchUserData(id)
    };
  }
  