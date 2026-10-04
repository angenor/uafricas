<template>
  <AfricansModale
    :model-value="modelValue"
    titre="Défier un ami"
    icone="fa-solid fa-hand-fist"
    @update:model-value="$emit('update:modelValue', $event)"
  >
    <form id="form-proposer-duel" class="flex flex-col gap-5" @submit.prevent="envoyer">
      <p class="text-[14px]/[1.5] text-af-corps">
        Vous jouez la même série, chacun quand il veut. Le résultat tombe quand vous avez joué tous
        les deux.
      </p>

      <!-- Ami -->
      <div>
        <p class="mb-2 text-[14px]/[1.4] font-bold text-af-encre">Qui défier ?</p>
        <p v-if="chargement" class="text-[14px]/[1.5] text-af-atone">Chargement de vos ami(e)s…</p>
        <p v-else-if="amis.length === 0" class="text-[14px]/[1.5] text-af-corps">
          On ne défie qu'un ami ou une amie, et vous n'en avez pas encore.
          <NuxtLink to="/profil" class="font-bold text-af-chocolat underline-offset-2 hover:underline">
            Trouver des membres
          </NuxtLink>
        </p>
        <ul v-else class="flex max-h-56 flex-col gap-1 overflow-y-auto" role="radiogroup" aria-label="Ami à défier">
          <li v-for="ami in amis" :key="ami.utilisateur.id">
            <label
              class="flex cursor-pointer items-center gap-3 rounded-lg border px-3 py-2 transition"
              :class="adversaireId === ami.utilisateur.id
                ? 'border-af-chocolat bg-af-chocolat/[0.07]'
                : 'border-af-bordure hover:border-af-chocolat'"
            >
              <input v-model="adversaireId" type="radio" :value="ami.utilisateur.id" class="accent-af-chocolat">
              <AfricansAvatar
                :nom="`${ami.utilisateur.prenom} ${ami.utilisateur.nom}`"
                :src="urlMedia(ami.utilisateur.photoUrl) ?? undefined"
                :taille="32"
              />
              <span class="min-w-0 flex-1 truncate text-[15px]/[1.4] text-af-encre">
                {{ ami.utilisateur.prenom }} {{ ami.utilisateur.nom }}
              </span>
            </label>
          </li>
        </ul>
      </div>

      <!-- Module -->
      <AfricansChamp v-model="module" libelle="Sur quel module ?" type="select" obligatoire>
        <option v-for="m in modulesJouables" :key="m.code" :value="m.code">{{ m.libelle }}</option>
      </AfricansChamp>

      <!-- Mode -->
      <fieldset>
        <legend class="mb-2 text-[14px]/[1.4] font-bold text-af-encre">Comment ?</legend>
        <div class="flex flex-col gap-2">
          <label
            v-for="option in MODES"
            :key="option.valeur"
            class="flex cursor-pointer items-start gap-3 rounded-lg border px-3 py-2 transition"
            :class="mode === option.valeur ? 'border-af-chocolat bg-af-chocolat/[0.07]' : 'border-af-bordure hover:border-af-chocolat'"
          >
            <input v-model="mode" type="radio" :value="option.valeur" class="mt-1 accent-af-chocolat" name="mode-duel">
            <span class="text-[14px]/[1.5] text-af-encre"><strong>{{ option.titre }}</strong> : {{ option.detail }}</span>
          </label>
        </div>
      </fieldset>

      <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
    </form>

    <template #actions>
      <button
        type="button"
        class="text-base font-bold text-af-chocolat transition hover:opacity-70"
        @click="$emit('update:modelValue', false)"
      >
        Annuler
      </button>
      <AfricansBouton
        icone="fa-solid fa-hand-fist"
        :desactive="!adversaireId || !module || envoi"
        @click="envoyer"
      >
        Lancer le défi
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import type { AmiAPI } from '~/composables/useAmis'
import { messageErreurJeu, type ModuleJeuAPI } from '~/composables/useJeu'
import type { DuelAPI } from '~/composables/useDuels'

const props = defineProps<{
  modelValue: boolean
  /** Module présélectionné (depuis la page d'un module). */
  moduleInitial?: string
  /** Ami présélectionné. */
  adversaireInitial?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [boolean]
  propose: [duel: DuelAPI & { sera_compte: boolean }]
}>()

const { listerAmis } = useAmis()
const { listerModules } = useJeu()
const { proposerDuel } = useDuels()

const amis = ref<AmiAPI[]>([])
const modules = ref<ModuleJeuAPI[]>([])
const adversaireId = ref('')
const module = ref('')
const chargement = ref(false)
const envoi = ref(false)
const erreur = ref('')
const mode = ref<'differe' | 'direct'>('differe')
const MODES = [
  { valeur: 'differe' as const, titre: 'En différé', detail: 'chacun joue quand il veut, dans les 48 heures.' },
  { valeur: 'direct' as const, titre: 'En direct', detail: 'au même moment, sur la même épreuve. Votre ami a quelques minutes pour accepter.' },
]

const modulesJouables = computed(() => modules.value.filter(m => m.disponible))

watch(() => props.modelValue, async (ouverte) => {
  if (!ouverte) return
  erreur.value = ''
  adversaireId.value = props.adversaireInitial ?? ''
  mode.value = 'differe'
  chargement.value = true
  try {
    const [a, m] = await Promise.all([listerAmis(), listerModules()])
    amis.value = a
    modules.value = m
    module.value = props.moduleInitial && modulesJouables.value.some(x => x.code === props.moduleInitial)
      ? props.moduleInitial
      : (modulesJouables.value[0]?.code ?? '')
  }
  finally {
    chargement.value = false
  }
}, { immediate: true })

const envoyer = async () => {
  if (!adversaireId.value || !module.value || envoi.value) return
  erreur.value = ''
  envoi.value = true
  try {
    const duel = await proposerDuel(adversaireId.value, module.value, mode.value)
    if (duel) {
      emit('propose', duel)
      emit('update:modelValue', false)
    }
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Le défi n\'a pas pu être envoyé.')
  }
  finally {
    envoi.value = false
  }
}
</script>
