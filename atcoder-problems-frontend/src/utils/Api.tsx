import { UserId } from "../interfaces/Status";
import Submission, { isSubmission } from "../interfaces/Submission";

const ATCODER_API_URL = process.env.REACT_APP_ATCODER_API_URL;

export const fetchRecentSubmissions = (): Promise<Submission[]> => {
  return fetch(`${ATCODER_API_URL}/v3/recent`)
    .then((r) => r.json())
    .then((r: unknown[]) => r.filter(isSubmission));
};

export const fetchPartialUserSubmissions = async (
  userId: UserId,
  fromSecond: number
): Promise<Submission[]> => {
  if (userId.length === 0) {
    return [];
  }
  const url = `${ATCODER_API_URL}/v3/user/submissions?user=${userId}&from_second=${fromSecond}`;
  const response = await fetch(url);

  const json: unknown = await response.json();
  if (!Array.isArray(json)) {
    return [];
  }
  return json.filter(isSubmission);
};
