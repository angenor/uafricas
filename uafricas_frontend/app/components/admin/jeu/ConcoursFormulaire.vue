<script setup lang="ts">
/**
 * Formulaire d'un concours (back-office, daisyUI).
 *
 * Une fois l'appel ouvert, seuls le thème, le règlement, le visuel et les dates
 * encore à venir restent modifiables (FR-024) : les autres champs sont
 * désactivés, et le serveur refuse de toute façon de les changer.
 */
import type { ConcoursForm, PhaseConcours } from '~/composables/useAdminConcours'
import { MODULES_PLATEFORME } from '~/utils/modulesPlateforme'

const form = defineModel<ConcoursForm>({ required: true })
const props = defineProps<{ phase?: PhaseConcours | null }>()

/** Avant l'ouverture de l'appel (ou à la création), tout se modifie. */
const libre = computed(() => !props.phase || props.phase === 'a_venir')
const maintenant = Date.now()
const dateFigee = (iso: string) => !libre.value && new Date(iso).getTime() <= maintenant

/** `datetime-local` ↔ ISO : le champ parle en heure locale, l'API en UTC. */
const versLocal = (iso: string) => {
  if (!iso) return ''
  const d = new Date(iso)
  return new Date(d.getTime() - d.getTimezoneOffset() * 60000).toISOString().slice(0, 16)
}
const champDate = (cle: 'appel_debut' | 'vote_debut' | 'vote_fin') => computed({
  get: () => versLocal(form.value[cle]),
  set: (v: string) => { form.value[cle] = v ? new Date(v).toISOString() : '' },
})
const appelDebut = champDate('appel_debut')
const voteDebut = champDate('vote_debut')
const voteFin = champDate('vote_fin')

/** Contrôle immédiat ; le serveur refait les mêmes. */
const alerteDates = computed(() => {
  const a = new Date(form.value.appel_debut).getTime()
  const v = new Date(form.value.vote_debut).getTime()
  const f = new Date(form.value.vote_fin).getTime()
  if (!a || !v || !f) return ''
  if (a >= v) return 'L\'appel à participation doit s\'ouvrir avant le vote.'
  if (f - v < 24 * 3600 * 1000) return 'Le vote dure au moins 24 heures.'
  return ''
})

const primesPodium = computed({
  get: () => form.value.prime_podium !== null,
  set: (v: boolean) => { form.value.prime_podium = v ? [30, 20, 10] : null },
})
</script>

<template>
  <fieldset class="space-y-6">
    <p v-if="!libre" class="alert alert-info text-sm">
      L'appel à participation est ouvert : seuls le thème, le règlement, le visuel et les dates encore à venir se modifient.
    </p>

    <div class="grid gap-4 md:grid-cols-3">
      <label class="flex flex-col md:col-span-2">
        <span class="label-text mb-1 font-medium">Titre *</span>
        <input v-model="form.titre" type="text" maxlength="150" class="input input-bordered input-sm w-full" :disabled="!libre" placeholder="Mon plat du dimanche" required>
      </label>
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Module de rattachement</span>
        <select v-model="form.rattachement" class="select select-bordered select-sm w-full" :disabled="!libre">
          <option :value="null">Aucun (espace Activités seulement)</option>
          <option v-for="m in MODULES_PLATEFORME" :key="m.code" :value="m.code">{{ m.libelle }}</option>
        </select>
      </label>
    </div>

    <label class="flex flex-col">
      <span class="label-text mb-1 font-medium">Thème *</span>
      <textarea v-model="form.theme" rows="2" class="textarea textarea-bordered w-full" placeholder="Le plat qui réunit votre famille le dimanche." required />
    </label>

    <label class="flex flex-col">
      <span class="label-text mb-1 font-medium">Règlement *</span>
      <textarea v-model="form.reglement" rows="4" class="textarea textarea-bordered w-full" placeholder="Une photo par membre, prise par vous, sans visage reconnaissable d'un mineur…" required />
      <span class="mt-1 text-xs text-base-content/60">
        Les règles d'admissibilité des voix (comptes récents, adresses non vérifiées, votes trop rapides) sont ajoutées automatiquement à la page publique.
      </span>
    </label>

    <div class="grid gap-4 md:grid-cols-3">
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Ouverture de l'appel *</span>
        <input v-model="appelDebut" type="datetime-local" class="input input-bordered input-sm w-full" :disabled="dateFigee(form.appel_debut)" required>
      </label>
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Ouverture du vote *</span>
        <input v-model="voteDebut" type="datetime-local" class="input input-bordered input-sm w-full" :disabled="dateFigee(form.vote_debut)" required>
      </label>
      <label class="flex flex-col">
        <span class="label-text mb-1 font-medium">Clôture du vote *</span>
        <input v-model="voteFin" type="datetime-local" class="input input-bordered input-sm w-full" :disabled="dateFigee(form.vote_fin)" required>
      </label>
    </div>
    <p v-if="alerteDates" class="text-sm text-error" role="alert">{{ alerteDates }}</p>

    <details class="collapse collapse-arrow border border-base-300 bg-base-100">
      <summary class="collapse-title text-sm font-medium">Réglages du concours</summary>
      <div class="collapse-content grid gap-4 md:grid-cols-3">
        <label class="flex flex-col">
          <span class="label-text mb-1">Photos par membre</span>
          <input v-model.number="form.participations_max" type="number" min="1" max="10" class="input input-bordered input-sm w-full" :disabled="!libre">
        </label>
        <label class="flex flex-col">
          <span class="label-text mb-1">Minimum de photos publiées pour ouvrir le vote</span>
          <input v-model.number="form.minimum_participations" type="number" min="2" class="input input-bordered input-sm w-full" :disabled="!libre">
        </label>
        <label class="flex flex-col">
          <span class="label-text mb-1">Plafond de votes par membre (vide : aucun)</span>
          <input v-model.number="form.votes_max" type="number" min="10" class="input input-bordered input-sm w-full" :disabled="!libre">
        </label>
        <label class="flex flex-col">
          <span class="label-text mb-1">Duels minimum pour accéder au podium</span>
          <input v-model.number="form.presentations_min" type="number" min="1" class="input input-bordered input-sm w-full" :disabled="!libre">
        </label>
        <label class="flex items-center gap-2 md:col-span-2">
          <input v-model="form.jury" type="checkbox" class="checkbox checkbox-sm" :disabled="!libre">
          <span class="label-text">Un jury fixe le podium parmi les finalistes du vote</span>
        </label>
        <template v-if="form.jury">
          <label class="flex flex-col">
            <span class="label-text mb-1">Finalistes soumis au jury</span>
            <input v-model.number="form.jury_finalistes" type="number" min="3" max="30" class="input input-bordered input-sm w-full" :disabled="!libre">
          </label>
          <label class="flex flex-col">
            <span class="label-text mb-1">Délai de délibération (jours)</span>
            <input v-model.number="form.jury_delai_jours" type="number" min="1" max="30" class="input input-bordered input-sm w-full" :disabled="!libre">
          </label>
        </template>
        <label class="flex flex-col">
          <span class="label-text mb-1">Prime de participation (vide : règle du jeu)</span>
          <input v-model.number="form.prime_participation" type="number" min="0" class="input input-bordered input-sm w-full" :disabled="!libre">
        </label>
        <label class="flex items-center gap-2 md:col-span-2">
          <input v-model="primesPodium" type="checkbox" class="checkbox checkbox-sm" :disabled="!libre">
          <span class="label-text">Primes de podium propres à ce concours</span>
        </label>
        <div v-if="form.prime_podium" class="flex gap-2 md:col-span-3">
          <label v-for="(_, i) in form.prime_podium" :key="i" class="flex flex-col">
            <span class="label-text mb-1">{{ i + 1 }}{{ i === 0 ? 're' : 'e' }} place</span>
            <input v-model.number="form.prime_podium[i]" type="number" min="0" class="input input-bordered input-sm w-28" :disabled="!libre">
          </label>
        </div>
      </div>
    </details>
  </fieldset>
</template>
