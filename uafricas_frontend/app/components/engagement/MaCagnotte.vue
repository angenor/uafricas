<script setup lang="ts">
/**
 * Cagnotte de soutien du membre : Tailwind v4 pur.
 *
 * Deux mentions y sont **obligatoires**, pas décoratives :
 * le versement n'est pas disponible : afficher un solde sans le dire
 *   laisserait croire à un retrait imminent ;
 * la part issue de paiements simulés, qui disparaîtra à la mise en service du
 *   paiement réel. La purge ne doit surprendre personne.
 */
import { onMounted, ref } from 'vue'
import { useCadeaux, formaterMontant, type Cagnotte } from '~/composables/useCadeaux'

const { obtenirMaCagnotte } = useCadeaux()
const cagnotte = ref<Cagnotte | null>(null)
const chargement = ref(true)

onMounted(async () => {
  try {
    cagnotte.value = await obtenirMaCagnotte()
  } catch {
    cagnotte.value = null
  } finally {
    chargement.value = false
  }
})
</script>

<template>
  <section v-if="!chargement && cagnotte" class="space-y-3">
    <h2 class="font-oswald text-xl font-bold text-gray-900">Ma cagnotte</h2>

    <div class="rounded-2xl border border-gray-100 bg-linear-to-br from-custom-chocolat/5 to-custom-chocolat/10 p-5">
      <p class="text-xs uppercase tracking-wide text-gray-500">Montant cumulé</p>
      <p class="mt-1 font-oswald text-4xl font-bold text-custom-chocolat">
        {{ formaterMontant(cagnotte.montant_cumule, cagnotte.devise) }}
      </p>
      <p class="mt-1 text-xs text-gray-500">
        Part des cadeaux « soutien financier » qui vous revient.
      </p>

      <!-- Un solde affiché sans cette mention se lit comme un solde retirable. -->
      <p
        v-if="!cagnotte.versement_disponible"
        class="mt-4 flex items-start gap-2 rounded-xl bg-white/70 px-3 py-2.5 text-xs leading-relaxed text-gray-600"
      >
        <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-0.5 shrink-0 text-gray-400" />
        <span>
          <strong>Le versement n'est pas encore disponible.</strong>
          Votre cagnotte s'accumule et sera versable dès la mise en service des
          paiements réels.
        </span>
      </p>

      <p
        v-if="cagnotte.part_simulee > 0"
        class="mt-2 flex items-start gap-2 rounded-xl bg-amber-50 px-3 py-2.5 text-xs leading-relaxed text-amber-900"
      >
        <font-awesome-icon
          icon="fa-solid fa-triangle-exclamation"
          class="mt-0.5 shrink-0 text-amber-600"
        />
        <span>
          Dont <strong>{{ formaterMontant(cagnotte.part_simulee, cagnotte.devise) }}</strong>
          issus de paiements simulés (phase de test). Ce montant, et les points
          correspondants, seront retirés à la mise en service du paiement réel.
        </span>
      </p>
    </div>
  </section>
</template>
