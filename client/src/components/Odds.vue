<script setup lang="ts">
import { reactive, ref, type Ref } from 'vue';
import {useLazyQuery} from '@vue/apollo-composable';
import GET_ODDS from '../graphql/getOdds';
import type { Empire } from '@/interfaces/empire';
import type { ApolloError } from '@apollo/client/errors';
import type { Response } from '@/interfaces/response';




defineProps<{
    result: Response | null ,
    loading: boolean,
    error: ApolloError | any,
}>()



</script>


<template>
    <div>
        <div v-if="result">the odds to the reach Endor in time is equal to {{result.getProbability}} %</div>
        <div v-else-if="loading">Loading...</div>
        <div v-else-if="error">Error... {{error.message}}</div>
        <div v-else-if="!result">Please upload the intercepted empire file</div>
    </div>
</template>