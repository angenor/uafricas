<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Mon compte', vers: '/mon-compte/profil' }, { libelle: 'Propositions médias' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <header>
        <h1 class="text-[24px]/[1.3] font-bold text-af-encre">Mes propositions de médias</h1>
        <p class="mt-1 text-[14px]/[1.5] text-af-corps">Suivez l’examen de vos propositions de chaînes, stations et émissions.</p>
      </header>
      <!-- Filtre -->
      <div class="flex flex-wrap gap-2 mb-6">
        <button
          type="button"
          class="rounded-full border px-4 py-1.5 text-sm font-medium transition-colors cursor-pointer"
          :class="filtreStatut === ''
            ? 'bg-gray-800 text-white border-gray-800'
            : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'"
          @click="filtreStatut = ''"
        >
          Toutes
        </button>
        <button
          v-for="(libelle, statut) in LIBELLES_STATUT"
          :key="statut"
          type="button"
          class="rounded-full border px-4 py-1.5 text-sm font-medium transition-colors cursor-pointer"
          :class="filtreStatut === statut
            ? 'bg-custom-chocolat text-white border-custom-chocolat'
            : 'bg-white text-gray-600 border-gray-300 hover:bg-gray-50'"
          @click="filtreStatut = statut"
        >
          {{ libelle }}
        </button>
      </div>

      <div v-if="chargement" class="flex justify-center py-16">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-custom-chocolat"></div>
      </div>

      <div v-else-if="propositions.length === 0" class="text-center py-16">
        <font-awesome-icon :icon="['fas', 'inbox']" class="w-12 h-12 text-gray-300 mb-4" />
        <p class="text-gray-600 mb-1">Vous n’avez encore proposé aucun média.</p>
        <p class="text-sm text-gray-400">
          Le bouton « Proposer un contenu » figure sur les pages Radio et Télé.
        </p>
      </div>

      <ul v-else class="space-y-4">
        <li
          v-for="proposition in propositions"
          :key="proposition.id"
          class="bg-white rounded-xl shadow-sm border border-gray-100 p-5"
        >
          <div class="flex items-start justify-between gap-4 flex-wrap mb-2">
            <div class="min-w-0">
              <h2 class="font-semibold text-gray-900">
                {{ proposition.donnees.nom || LIBELLES_TYPE_OBJET[proposition.type_objet] }}
              </h2>
              <p class="text-xs text-gray-400 mt-0.5">
                {{ LIBELLES_TYPE_OBJET[proposition.type_objet] }} ·
                proposée le {{ dateFormatee(proposition.created_at) }}
              </p>
            </div>
            <span
              class="shrink-0 rounded-full px-3 py-1 text-xs font-bold uppercase tracking-wide"
              :class="STYLES_STATUT[proposition.statut]"
            >
              {{ LIBELLES_STATUT[proposition.statut] }}
            </span>
          </div>

          <p v-if="proposition.donnees.description" class="text-sm text-gray-600 line-clamp-2 mb-3">
            {{ proposition.donnees.description }}
          </p>

          <!-- Le motif du refus est l'information la plus utile de cet écran. -->
          <div
            v-if="proposition.statut === 'rejetee' && proposition.commentaire_decision"
            class="rounded-lg bg-red-50 border border-red-200 px-4 py-3 text-sm text-red-900 mb-3"
          >
            <span class="font-semibold">Motif du refus :</span>
            {{ proposition.commentaire_decision }}
          </div>

          <div
            v-else-if="proposition.statut === 'validee'"
            class="rounded-lg bg-green-50 border border-green-200 px-4 py-3 text-sm text-green-900 mb-3"
          >
            Publiée sur le site.
            <span v-if="proposition.commentaire_decision">
              Commentaire de l’administrateur : {{ proposition.commentaire_decision }}
            </span>
          </div>

          <!-- Le retrait n'a de sens que tant que rien n'a été tranché. -->
          <button
            v-if="proposition.statut === 'en_attente'"
            type="button"
            class="text-sm text-red-600 hover:underline cursor-pointer"
            @click="retirerProposition(proposition)"
          >
            Retirer cette proposition
          </button>
        </li>
      </ul>
    </div>

    <template #rail>
      <ComptePanneauNavigation />
    </template>
  </NuxtLayout>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
