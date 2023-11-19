import type { BountyHunter } from '../types/hunter';


// Empire is an object with two properties:
//
// countdown: number
// bounty_hunters: Hunter[]
//
//
export interface Empire  {
    countdown: number;
    bountyHunters: BountyHunter[];
}