<template>
  <div class="flex flex-col gap-6">
    <section v-for="groupe in groupes" :key="groupe.cle" v-show="groupe.duels.length > 0 || groupe.toujours">
      <h3 class="mb-3 text-[15px]/[1.4] font-bold text-af-encre">
        {{ groupe.titre }} <span class="font-normal text-af-atone">({{ groupe.duels.length }})</span>
      </h3>
      <p v-if="groupe.duels.length === 0" class="text-[14px]/[1.5] text-af-atone">{{ groupe.vide }}</p>
      <ul v-else class="flex flex-col gap-2">
        <li
          v-for="duel in groupe.duels"
          :key="duel.id"
          class="flex flex-wrap items-center gap-3 rounded-[10px] border border-af-bordure bg-af-surface px-4 py-3"
        >
          <AfricansAvatar
            :nom="nomAdversaire(duel)"
            :src="urlMedia(duel.adversaire?.photoUrl) ?? undefined"
            :taille="36"
          />
          <NuxtLink :to="`/activites/duels/${duel.id}`" class="min-w-0 flex-1">
            <span class="block truncate text-[15px]/[1.4] font-bold text-af-encre hover:text-af-chocolat">
              {{ nomAdversaire(duel) }}
            </span>
            <span class="block truncate text-[13px]/[1.4] text-af-atone">
              {{ duel.module_libelle }}
              <template v-if="!duel.compte"> · amical, sans gain</template>
                <template v-if="duel.mode === 'direct'"> · en direct</template>
              <template v-if="groupe.cle !== 'termines'"> · {{ echeance(duel) }}</template>
            </span>
          </NuxtLink>

          <!-- Actions selon l'étape -->
          <template v-if="groupe.cle === 'a_repondre'">
            <AfricansBouton variante="secondaire" :desactive="occupe" @click="$emit('refuser', duel)">Refuser</AfricansBouton>
            <AfricansBouton icone="fa-solid fa-check" :desactive="occupe" @click="$emit('accepter', duel)">Accepter</AfricansBouton>
          </template>
          <AfricansBouton
            v-else-if="groupe.cle === 'a_jouer' && duel.mode === 'direct'"
            icone="fa-solid fa-bolt"
            :vers="`/activites/duels/${duel.id}`"
          >
            Rejoindre la salle
          </AfricansBouton>
          <AfricansBouton
            v-else-if="groupe.cle === 'a_jouer'"
            icone="fa-solid fa-play"
            :desactive="occupe"
            @click="$emit('jouer', duel)"
          >
            {{ duel.ma_partie ? 'Reprendre' : 'Jouer' }}
          </AfricansBouton>
          <span
            v-else-if="groupe.cle === 'termines'"
            class="text-[14px]/[1.4] font-bold"
            :class="duel.vainqueur_id === moiId ? 'text-af-vert' : 'text-af-atone'"
          >
            {{ resultatDuel(duel, moiId) }}
            <template v-if="duel.mon_gain > 0"> · +{{ duel.mon_gain }}</template>
          </span>
          <span v-else class="text-[13px]/[1.4] text-af-atone">
            {{ duel.etat === 'propose' ? 'En attente de réponse' : 'En attente de son jeu' }}
          </span>
        </li>
      </ul>
    </section>
  </div>
</template>

<script setup lang="ts">
import { dureeRestante } from '~/composables/useJeu'
import { resultatDuel, type DuelAPI, type MesDuelsAPI } from '~/composables/useDuels'

const props = defineProps<{
  duels: MesDuelsAPI
  moiId: string | null
  occupe?: boolean
}>()

defineEmits<{ accepter: [DuelAPI], refuser: [DuelAPI], jouer: [DuelAPI] }>()

const groupes = computed(() => [
  { cle: 'a_repondre', titre: 'On vous défie', duels: props.duels.a_repondre, vide: '', toujours: false },
  { cle: 'a_jouer', titre: 'À vous de jouer', duels: props.duels.a_jouer, vide: '', toujours: false },
  { cle: 'en_attente', titre: 'En attente', duels: props.duels.en_attente, vide: '', toujours: false },
  { cle: 'termines', titre: 'Terminés', duels: props.duels.termines, vide: 'Aucun duel terminé pour le moment.', toujours: true },
])

const nomAdversaire = (duel: DuelAPI) =>
  duel.adversaire ? `${duel.adversaire.prenom} ${duel.adversaire.nom}` : 'Membre'

const echeance = (duel: DuelAPI) => `plus que ${dureeRestante(duel.echeance_at)}`
</script>
