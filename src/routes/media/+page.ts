export async function load({ params, url }) {
  let id = url.searchParams.get("id");
  return { id };
}
