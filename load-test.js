import http from 'k6/http';
import { check } from 'k6';

export const options = {
  // We will aggressively ramp up to find the breaking point
  stages: [
    { duration: '30s', target: 1000 },
    { duration: '30s', target: 2000 },
    { duration: '30s', target: 3000 },
    { duration: '30s', target: 4000 },
    { duration: '30s', target: 5000 },
  ],
};

export default function () {
  const res = http.get('http://127.0.0.1:3000/');
  check(res, { 'status is 200': (r) => r.status === 200 });
}
