<template>
  <Transition name="modal-fade">
    <div
      v-if="open"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-xs"
      @click.self="$emit('close')"
    >
      <div
        class="relative w-full max-w-2xl max-h-[90vh] flex flex-col bg-white shadow-2xl rounded-3xl overflow-hidden"
        @click.stop
      >
        <!-- En-tête -->
        <div class="relative shrink-0 bg-linear-to-r from-custom-chocolat to-custom-chocolat/80 px-6 py-6 text-white">
          <button
            type="button"
            class="absolute top-4 right-4 text-white/80 hover:text-white transition-colors"
            aria-label="Fermer"
            @click="$emit('close')"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="w-5 h-5" />
          </button>

          <div class="flex items-center gap-3">
            <div class="w-12 h-12 rounded-2xl bg-white/15 flex items-center justify-center shrink-0">
              <font-awesome-icon :icon="['fas', 'users']" class="w-6 h-6" />
            </div>
            <div>
              <h2 class="text-xl md:text-2xl font-bold leading-tight">Bibliothèques Humaines</h2>
              <p class="text-white/90 text-sm">Rencontrer celles et ceux qui portent la mémoire du continent</p>
            </div>
          </div>
        </div>

        <!-- Corps défilant -->
        <div class="overflow-y-auto px-6 py-6 space-y-8">
          <!-- Le pourquoi -->
          <p class="text-gray-700 leading-relaxed">
            En Afrique, on dit qu'un vieillard qui meurt est une bibliothèque qui brûle.
            <strong class="text-gray-900">HumanTech</strong> relie celles et ceux qui détiennent
            un savoir — aînés, griots, experts, témoins de l'histoire — à celles et ceux qui
            veulent apprendre. Ici, on ne prête pas des livres&nbsp;: on
            <strong class="text-gray-900">écoute des personnes</strong>, on recueille leurs
            expériences de vie et on garde vivante la mémoire du continent et de ses diasporas.
          </p>

          <!-- Ce que vous pouvez faire -->
          <div>
            <h3 class="text-sm font-bold uppercase tracking-wide text-custom-chocolat mb-4">
              Ce que vous pouvez y faire
            </h3>
            <div class="grid sm:grid-cols-2 gap-3">
              <div
                v-for="item in fonctionnalites"
                :key="item.titre"
                class="flex gap-3 rounded-2xl border border-gray-100 bg-gray-50/60 p-4"
              >
                <div class="w-10 h-10 rounded-xl bg-custom-green/10 text-custom-green flex items-center justify-center shrink-0">
                  <font-awesome-icon :icon="['fas', item.icone]" class="w-5 h-5" />
                </div>
                <div class="min-w-0">
                  <p class="font-semibold text-gray-900 text-sm">{{ item.titre }}</p>
                  <p class="text-gray-500 text-xs mt-0.5 leading-relaxed">{{ item.texte }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Les outils -->
          <div>
            <h3 class="text-sm font-bold uppercase tracking-wide text-custom-chocolat mb-4">
              Les outils à votre disposition
            </h3>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="outil in outils"
                :key="outil"
                class="inline-flex items-center gap-1.5 rounded-full bg-custom-chocolat/5 text-custom-chocolat text-xs font-medium px-3 py-1.5"
              >
                <font-awesome-icon :icon="['fas', 'circle-check']" class="w-3 h-3 text-custom-green" />
                {{ outil }}
              </span>
            </div>
          </div>

          <!-- Notre objectif -->
          <div class="rounded-2xl bg-custom-green/5 border border-custom-green/15 p-5">
            <h3 class="flex items-center gap-2 text-sm font-bold text-custom-green mb-2">
              <font-awesome-icon :icon="['fas', 'seedling']" class="w-4 h-4" />
              Notre objectif
            </h3>
            <p class="text-gray-700 text-sm leading-relaxed">
              Préserver le patrimoine immatériel vivant de l'Afrique, transmettre directement les
              savoirs et les expériences, et donner toute leur place à celles et ceux qui portent
              ces mémoires.
            </p>
          </div>
        </div>

        <!-- Pied -->
        <div class="shrink-0 border-t border-gray-100 px-6 py-4 bg-gray-50/50">
          <button
            type="button"
            class="w-full sm:w-auto sm:ml-auto sm:block px-6 py-2.5 rounded-full bg-custom-chocolat text-white font-semibold text-sm hover:bg-custom-chocolat/90 transition-colors"
            @click="$emit('close')"
          >
            J'ai compris
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
defineProps<{ open: boolean }>()
defineEmits<{ close: [] }>()

const fonctionnalites = [
  {
    icone: 'user-tie',
    titre: 'Devenir bibliothèque humaine',
    texte: 'Créez votre profil et présentez les domaines et expériences que vous souhaitez transmettre.',
  },
  {
    icone: 'id-card',
    titre: 'Consulter les profils',
    texte: 'Parcourez les biographies, les spécialités et les domaines de nos personnes ressources.',
  },
  {
    icone: 'magnifying-glass',
    titre: 'Rechercher une personne ressource',
    texte: 'Trouvez la bonne personne grâce à la recherche et aux filtres par spécialité.',
  },
  {
    icone: 'comments',
    titre: 'Réagir & recommander',
    texte: 'Aimez, commentez et recommandez les personnes ressources dont le savoir vous a marqué.',
  },
]

const outils = [
  'Répertoire de personnes ressources',
  'Recherche par spécialité',
  'Réactions & recommandations',
]
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
