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
        <div class="relative shrink-0 bg-linear-to-r from-af-chocolat to-af-chocolat/80 px-6 py-6 text-white">
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
              <font-awesome-icon :icon="['fas', 'shield-halved']" class="w-6 h-6" />
            </div>
            <div>
              <h2 class="text-xl md:text-2xl font-bold leading-tight">Règles de contenu</h2>
              <p class="text-white/90 text-sm">Ce qui n'a pas sa place sur nos antennes</p>
            </div>
          </div>
        </div>

        <!-- Corps défilant -->
        <div class="overflow-y-auto px-6 py-6 space-y-8">
          <p class="text-af-corps leading-relaxed">
            Les antennes Télé et Radio d'Africans sont ouvertes à toutes les voix du continent.
            Cette ouverture suppose une exigence :
            <strong class="text-af-encre">aucun contenu ne peut porter atteinte aux personnes,
            aux peuples ou à la bonne gouvernance</strong>.
          </p>

          <!-- Les contenus interdits -->
          <div>
            <h3 class="text-sm font-bold uppercase tracking-wide text-af-chocolat mb-4">
              Contenus interdits
            </h3>
            <div class="grid sm:grid-cols-2 gap-3">
              <div
                v-for="regle in reglesInterdites"
                :key="regle.titre"
                class="rounded-2xl border border-af-bordure bg-af-fond/60 p-4"
              >
                <div class="flex items-start gap-3">
                  <div class="w-10 h-10 rounded-xl bg-af-live/5 text-af-live flex items-center justify-center shrink-0">
                    <font-awesome-icon :icon="['fas', regle.icone]" class="w-4 h-4" />
                  </div>
                  <div>
                    <p class="font-semibold text-af-encre text-sm">{{ regle.titre }}</p>
                    <p class="text-af-corps text-sm mt-0.5 leading-snug">{{ regle.detail }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Ce qui est attendu -->
          <div>
            <h3 class="text-sm font-bold uppercase tracking-wide text-af-chocolat mb-4">
              Ce que nous attendons
            </h3>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="attente in attentes"
                :key="attente"
                class="inline-flex items-center gap-1.5 rounded-full bg-af-chocolat/5 text-af-chocolat text-xs px-3 py-1.5"
              >
                <font-awesome-icon :icon="['fas', 'circle-check']" class="w-3 h-3" />
                {{ attente }}
              </span>
            </div>
          </div>

          <!-- Ce qu'il advient d'un signalement -->
          <div class="rounded-2xl bg-af-vert/5 border border-af-vert/15 p-5">
            <div class="flex items-start gap-3">
              <font-awesome-icon :icon="['fas', 'flag']" class="w-5 h-5 text-af-vert shrink-0 mt-0.5" />
              <div>
                <p class="font-semibold text-af-encre text-sm mb-1">Signaler un contenu</p>
                <p class="text-af-corps text-sm leading-relaxed">
                  Tout membre connecté peut signaler un contenu, une fois par contenu.
                  Au-delà d'un certain nombre de signalements distincts, le contenu est
                  <strong class="text-af-encre">automatiquement retiré de l'antenne</strong>
                  le temps qu'un administrateur l'examine : il peut alors le rétablir ou le
                  supprimer définitivement.
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- Pied -->
        <div class="shrink-0 border-t border-af-bordure px-6 py-4 bg-af-fond/50 flex justify-end">
          <button
            type="button"
            class="rounded-full bg-af-chocolat text-white px-6 py-2.5 text-sm font-semibold hover:bg-af-chocolat/90 transition-colors cursor-pointer"
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

/** Les cinq interdits énoncés par la spécification (FR-048), plus les droits. */
const reglesInterdites: { icone: string, titre: string, detail: string }[] = [
  {
    icone: 'hand-fist',
    titre: 'Violence',
    detail: 'Apologie, incitation ou banalisation de la violence sous toutes ses formes.',
  },
  {
    icone: 'ban',
    titre: 'Racisme',
    detail: 'Propos racistes, xénophobes ou attisant la haine entre peuples.',
  },
  {
    icone: 'users-slash',
    titre: 'Discrimination',
    detail: 'Contenus discriminatoires liés au genre, à l\'origine, à la religion ou au handicap.',
  },
  {
    icone: 'scale-unbalanced',
    titre: 'Mauvaise gouvernance',
    detail: 'Valorisation de pratiques contraires à l\'intérêt général et à l\'État de droit.',
  },
  {
    icone: 'sack-dollar',
    titre: 'Corruption',
    detail: 'Promotion, justification ou banalisation de la corruption.',
  },
  {
    icone: 'copyright',
    titre: 'Diffusion non autorisée',
    detail: 'Rediffusion d\'une œuvre sans en détenir les droits.',
  },
]

const attentes: string[] = [
  'Des sources identifiables',
  'Le respect des personnes',
  'Un propos vérifiable',
  'Des droits de diffusion détenus',
  'Un ton constructif',
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
