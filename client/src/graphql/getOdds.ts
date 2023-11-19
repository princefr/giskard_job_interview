import {gql} from '@apollo/client/core';


// Query for the odds of a successful melenium falcon reaching Endor
const GET_ODDS = gql`
    query GetProbability($empire: Empire!) {
        getProbability(empire: $empire)
    }
`


export default GET_ODDS;