<template>
  <section class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center">
    <font-awesome-icon icon="fa-solid fa-trophy" class="text-4xl text-af-chocolat" />
    <h2 class="mt-4 text-[24px]/[1.3] font-bold text-af-encre">
      <template v-if="partie.cadre === 'defi'">Défi terminé</template>
      <template v-else-if="partie.cadre === 'duel'">Votre partie de duel est jouée</template>
      <template v-else>{{ partie.etat === 'close' ? 'Partie interrompue' : 'Partie terminée' }}</template>
    </h2>

    <p v-if="partie.cadre === 'defi'" class="mt-2 text-[14px]/[1.5] text-af-atone">
      Le score gagné comprend la prime d'achèvement du défi.
    </p>

    <p v-if="partie.cadre === 'duel'" class="mt-2 text-[14px]/[1.5] text-af-atone">
      En duel, les réponses ne rapportent rien : c'est le résultat du duel qui compte.
    </p>

    <p v-if="partie.cadre === 'entrainement'" class="mt-2 text-[14px]/[1.5] text-af-atone">
      C'était un entraînement : il ne rapporte pas de score.
    </p>

    <dl class="mx-auto mt-6 grid max-w-md gap-4" :class="partie.cadre === 'duel' ? 'max-w-[10rem] grid-cols-1' : 'grid-cols-3'">
      <div class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Bonnes réponses</dt>
        <dd class="mt-1 text-[22px]/[1.2] font-bold text-af-encre">
          {{ partie.bonnes }}<span class="text-[14px] font-normal text-af-atone"> / {{ partie.nombre_epreuves }}</span>
        </dd>
      </div>
      <div v-if="partie.cadre !== 'duel'" class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Score gagné</dt>
        <dd class="mt-1 text-[22px]/[1.2] font-bold text-af-vert">+{{ partie.score_gagne }}</dd>
      </div>
      <div v-if="partie.cadre !== 'duel'" class="rounded-lg bg-af-fond p-4">
        <dt class="text-[13px]/[1.4] text-af-atone">Score total</dt>
        <dd class="mt-1 text-[22px]/[1.2] font-bold text-af-encre">{{ partie.score_total ?? 0 }}</dd>
      </div>
    </dl>

    <p v-if="partie.rang_saison" class="mt-4 text-[14px]/[1.5] text-af-corps">
      Vous êtes <strong class="text-af-chocolat">{{ partie.rang_saison }}<sup>{{ partie.rang_saison === 1 ? 'er' : 'e' }}</sup></strong>
      au Championship.
      <NuxtLink to="/activites/championship" class="font-bold text-af-chocolat underline-offset-2 hover:underline">
        Voir le classement
      </NuxtLink>
    </p>

    <!-- Le fil des réponses, dans l'ordre : il dit où la partie s'est jouée. -->
    <ol
      v-if="partie.reponses?.length"
      class="mt-6 flex flex-wrap justify-center gap-2"
      aria-label="Détail des réponses"
    >
      <li
        v-for="reponse in partie.reponses"
        :key="reponse.rang"
        class="grid size-8 place-items-center rounded-full text-[13px] font-bold"
        :class="classesIssue[reponse.issue]"
        :title="libellesIssue[reponse.issue]"
      >
        {{ reponse.rang }}
        <span class="sr-only">: {{ libellesIssue[reponse.issue] }}</span>
      </li>
    </ol>

    <!-- Un défi n'a qu'un essai : pas de « Rejouer », mais un classement. -->
    <div v-if="partie.cadre === 'defi'" class="mt-8 flex flex-wrap justify-center gap-4">
      <AfricansBouton icone="fa-solid fa-ranking-star" :vers="`/activites/defis/${partie.defi_id}`">
        Voir le classement du défi
      </AfricansBouton>
      <AfricansBouton variante="secondaire" vers="/activites">
        Retour aux activités
      </AfricansBouton>
    </div>
    <div v-else-if="partie.cadre === 'duel'" class="mt-8 flex flex-wrap justify-center gap-4">
      <AfricansBouton icone="fa-solid fa-hand-fist" :vers="`/activites/duels/${partie.duel_id}`">
        Voir le duel
      </AfricansBouton>
      <AfricansBouton variante="secondaire" vers="/activites/duels">Mes duels</AfricansBouton>
    </div>
    <div v-else class="mt-8 flex flex-wrap justify-center gap-4">
      <AfricansBouton v-if="partie.module" icone="fa-solid fa-rotate-right" @click="$emit('rejouer')">
        Rejouer
      </AfricansBouton>
      <AfricansBouton variante="secondaire" :vers="partie.module ? `/activites/${partie.module}` : '/activites'">
        Retour {{ partie.module ? 'au module' : 'aux activités' }}
      </AfricansBouton>
    </div>
  </section>
</template>

<script setup lang="ts">
import type { IssueReponse, PartieAPI } from '~/composables/useJeu'

defineProps<{ partie: PartieAPI }>()
defineEmits<{ rejouer: [] }>()

const classesIssue: Record<IssueReponse, string> = {
  bonne: 'bg-af-vert text-white',
  mauvaise: 'bg-af-live text-white',
  sans_reponse: 'bg-af-bordure text-af-corps',
  injouable: 'border border-dashed border-af-bordure text-af-atone',
}

const libellesIssue: Record<IssueReponse, string> = {
  bonne: 'bonne réponse',
  mauvaise: 'mauvaise réponse',
  sans_reponse: 'sans réponse',
  injouable: 'épreuve passée',
}
</script>
