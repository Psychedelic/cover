import {request} from "@octokit/request";

// @ts-ignore
const listUserRepositories = async ({access_token, owner, repo}) => {
    const ret = await request(// "GET /repos/{owner}/{repo}",
        "GET /user/repos",
        {
            headers: {authorization: `token ${access_token}`},
            owner,
            repo
        });
    return ret.data;
}

const filterRepos = (element: any, owner: String, repo: String) =>
    element.full_name.localeCompare(`${owner}/${repo}`, undefined, {sensitivity: 'accent'}) === 0;

/**
 * Check if user with access_token has access to git_repo
 * @param data includes { git_repo, access_token }
 * @param git_repo of kind "owner/repo"
 * @param access_token: Personal Access Token from user's Github development settings
 * @return true if the user has access to the repo
 */
const validRepoAccessToken = async (data: { git_repo: String, access_token: String }): Promise<boolean> => {
    const {git_repo, access_token} = data;
    const [owner, repo] = git_repo.split("/");

    const userRepos = await listUserRepositories({access_token, owner, repo});
    const hasRepo = userRepos.filter(el => filterRepos(el, owner, repo));

    // we shall find exactly one repo
    if (hasRepo.length != 1) return false;
    const {permissions} = hasRepo[0];
    const {admin, maintain, push} = permissions as { admin: boolean; maintain: boolean; push: boolean };

    // user must have at least one of these roles
    return admin || maintain || push;
}

export {validRepoAccessToken};
