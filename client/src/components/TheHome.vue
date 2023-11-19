<script setup lang="ts">

    import File from './File.vue';
    import Odds from './Odds.vue';
    import { ref, reactive, inject } from 'vue';
    import type { Empire } from '@/interfaces/empire';
    import {useLazyQuery} from '@vue/apollo-composable'; 
    import GET_ODDS from '@/graphql/getOdds';
    import type { BountyHunter } from '@/types/hunter';
    import type { Response } from '@/interfaces/response';
    
    
    const variables = reactive({
        empire: ref<Empire>({
            countdown: 0,
            bountyHunters: [],
        }),
    });
    const {result, loading, error, load} = useLazyQuery(GET_ODDS, variables);



    const getOddsFromTheRebellionDatabase = (empire: Empire) => {
        variables.empire.countdown = empire.countdown;
        for (let i = 0; i < empire.bountyHunters.length; i++) {
            const bountyHunter: BountyHunter = {
            planet: empire.bountyHunters[i].planet,
            day: empire.bountyHunters[i].day,
            };
        variables.empire.bountyHunters.push(bountyHunter);
        }
        load()

    }


    const oddsRef = ref(null);

    // handle file change function
    const handleFileChange = (event: Event) => {
        const target = event.target as HTMLInputElement;
        const files = target.files as FileList;
        // only one file allowed
        if (files.length > 1) {
            alert('Please upload only one file');
            return;
        }
        // check if the file is a json file
        if (files[0].type === 'application/json') {
            // read the file
            const reader = new FileReader();
            reader.readAsText(files[0], 'UTF-8');
            // handle the file
            reader.onload = (readerEvent) => {
                const fileContent = readerEvent.target?.result as string;
                const fileJson = JSON.parse(fileContent);
                if (fileJson.countdown === undefined || fileJson.bounty_hunters === undefined) {
                    alert('Please upload a valid empire json file');
                    return;
                }
                const empire: Empire = {
                    countdown: fileJson.countdown,
                    bountyHunters: fileJson.bounty_hunters
                };
                getOddsFromTheRebellionDatabase(empire);
            };
        } else {
            alert('Please upload a json file');
        }
    }



</script>


<template>
    <div class="flex flex-col justify-center items-center h-screen w-screen">
        <div class="flex flex-col space-y-6 items-center justify-center p-4">
            <File :handleFileChange="handleFileChange"></File>
            <Odds :result="result" :loading="loading" :error="error" :loadRebellionData="getOddsFromTheRebellionDatabase"/>
        </div>
    </div>
</template>